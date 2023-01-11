const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CancelMigration {
        pub vault_proxy: Vec<u8>,
        pub bypass_failure: bool,
    }

    impl CancelMigration {
        const METHOD_ID: [u8; 4] = [156u8, 157u8, 72u8, 218u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Address, ethabi::ParamType::Bool],
                maybe_data.unwrap(),
            )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                bypass_failure: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.vault_proxy)),
                ethabi::Token::Bool(self.bypass_failure),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for CancelMigration {
        const NAME: &'static str = "cancelMigration";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ClaimOwnership {}

    impl ClaimOwnership {
        const METHOD_ID: [u8; 4] = [78u8, 113u8, 224u8, 200u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for ClaimOwnership {
        const NAME: &'static str = "claimOwnership";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DeployVaultProxy {
        pub vault_lib: Vec<u8>,
        pub owner: Vec<u8>,
        pub vault_accessor: Vec<u8>,
        pub fund_name: String,
    }

    impl DeployVaultProxy {
        const METHOD_ID: [u8; 4] = [34u8, 160u8, 192u8, 139u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::String,
                ],
                maybe_data.unwrap(),
            )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                fund_name: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.vault_lib)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.vault_accessor)),
                ethabi::Token::String(self.fund_name.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for DeployVaultProxy {
        const NAME: &'static str = "deployVaultProxy";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for DeployVaultProxy {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ExecuteMigration {
        pub vault_proxy: Vec<u8>,
        pub bypass_failure: bool,
    }

    impl ExecuteMigration {
        const METHOD_ID: [u8; 4] = [56u8, 179u8, 235u8, 27u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Address, ethabi::ParamType::Bool],
                maybe_data.unwrap(),
            )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                bypass_failure: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.vault_proxy)),
                ethabi::Token::Bool(self.bypass_failure),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for ExecuteMigration {
        const NAME: &'static str = "executeMigration";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetCurrentFundDeployer {}

    impl GetCurrentFundDeployer {
        const METHOD_ID: [u8; 4] = [124u8, 119u8, 178u8, 237u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetCurrentFundDeployer {
        const NAME: &'static str = "getCurrentFundDeployer";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GetCurrentFundDeployer {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetFundDeployerForVaultProxy {
        pub vault_proxy: Vec<u8>,
    }

    impl GetFundDeployerForVaultProxy {
        const METHOD_ID: [u8; 4] = [61u8, 124u8, 116u8, 248u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.vault_proxy,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetFundDeployerForVaultProxy {
        const NAME: &'static str = "getFundDeployerForVaultProxy";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GetFundDeployerForVaultProxy {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetMigrationRequestDetailsForVaultProxy {
        pub vault_proxy: Vec<u8>,
    }

    impl GetMigrationRequestDetailsForVaultProxy {
        const METHOD_ID: [u8; 4] = [125u8, 173u8, 159u8, 200u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.vault_proxy,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, substreams::scalar::BigInt), String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(
            data: &[u8],
        ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, substreams::scalar::BigInt), String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                ],
                data.as_ref(),
            )
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            values.reverse();
            Ok((
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            ))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(
            &self,
            address: Vec<u8>,
        ) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>, substreams::scalar::BigInt)> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetMigrationRequestDetailsForVaultProxy {
        const NAME: &'static str = "getMigrationRequestDetailsForVaultProxy";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl
    substreams_ethereum::rpc::RPCDecodable<(
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        substreams::scalar::BigInt,
    )> for GetMigrationRequestDetailsForVaultProxy
    {
        fn output(
            data: &[u8],
        ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, substreams::scalar::BigInt), String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetMigrationTimelock {}

    impl GetMigrationTimelock {
        const METHOD_ID: [u8; 4] = [47u8, 160u8, 193u8, 97u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<substreams::scalar::BigInt, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok({
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            })
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetMigrationTimelock {
        const NAME: &'static str = "getMigrationTimelock";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt> for GetMigrationTimelock {
        fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetNominatedOwner {}

    impl GetNominatedOwner {
        const METHOD_ID: [u8; 4] = [40u8, 139u8, 106u8, 54u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetNominatedOwner {
        const NAME: &'static str = "getNominatedOwner";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GetNominatedOwner {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetOwner {}

    impl GetOwner {
        const METHOD_ID: [u8; 4] = [137u8, 61u8, 32u8, 232u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetOwner {
        const NAME: &'static str = "getOwner";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GetOwner {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetSharesTokenSymbol {}

    impl GetSharesTokenSymbol {
        const METHOD_ID: [u8; 4] = [180u8, 123u8, 6u8, 0u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<String, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<String, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_string()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<String> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetSharesTokenSymbol {
        const NAME: &'static str = "getSharesTokenSymbol";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<String> for GetSharesTokenSymbol {
        fn output(data: &[u8]) -> Result<String, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetTimelockRemainingForMigrationRequest {
        pub vault_proxy: Vec<u8>,
    }

    impl GetTimelockRemainingForMigrationRequest {
        const METHOD_ID: [u8; 4] = [119u8, 168u8, 198u8, 36u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.vault_proxy,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<substreams::scalar::BigInt, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok({
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            })
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for GetTimelockRemainingForMigrationRequest {
        const NAME: &'static str = "getTimelockRemainingForMigrationRequest";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
    for GetTimelockRemainingForMigrationRequest
    {
        fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HasExecutableMigrationRequest {
        pub vault_proxy: Vec<u8>,
    }

    impl HasExecutableMigrationRequest {
        const METHOD_ID: [u8; 4] = [102u8, 35u8, 28u8, 234u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.vault_proxy,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for HasExecutableMigrationRequest {
        const NAME: &'static str = "hasExecutableMigrationRequest";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<bool> for HasExecutableMigrationRequest {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HasMigrationRequest {
        pub vault_proxy: Vec<u8>,
    }

    impl HasMigrationRequest {
        const METHOD_ID: [u8; 4] = [208u8, 68u8, 157u8, 61u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.vault_proxy,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }

    impl substreams_ethereum::Function for HasMigrationRequest {
        const NAME: &'static str = "hasMigrationRequest";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    impl substreams_ethereum::rpc::RPCDecodable<bool> for HasMigrationRequest {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RemoveNominatedOwner {}

    impl RemoveNominatedOwner {
        const METHOD_ID: [u8; 4] = [129u8, 86u8, 238u8, 207u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for RemoveNominatedOwner {
        const NAME: &'static str = "removeNominatedOwner";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCurrentFundDeployer {
        pub next_fund_deployer: Vec<u8>,
    }

    impl SetCurrentFundDeployer {
        const METHOD_ID: [u8; 4] = [151u8, 175u8, 112u8, 80u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                next_fund_deployer: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.next_fund_deployer,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for SetCurrentFundDeployer {
        const NAME: &'static str = "setCurrentFundDeployer";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetMigrationTimelock {
        pub next_timelock: substreams::scalar::BigInt,
    }

    impl SetMigrationTimelock {
        const METHOD_ID: [u8; 4] = [29u8, 244u8, 25u8, 247u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values =
                ethabi::decode(&[ethabi::ParamType::Uint(256usize)], maybe_data.unwrap())
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                next_timelock: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                self.next_timelock.clone().to_signed_bytes_be().as_slice(),
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for SetMigrationTimelock {
        const NAME: &'static str = "setMigrationTimelock";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetNominatedOwner {
        pub next_nominated_owner: Vec<u8>,
    }

    impl SetNominatedOwner {
        const METHOD_ID: [u8; 4] = [114u8, 142u8, 23u8, 160u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                next_nominated_owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(
                &self.next_nominated_owner,
            ))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for SetNominatedOwner {
        const NAME: &'static str = "setNominatedOwner";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetSharesTokenSymbol {
        pub next_symbol: String,
    }

    impl SetSharesTokenSymbol {
        const METHOD_ID: [u8; 4] = [117u8, 123u8, 192u8, 221u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::String], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                next_symbol: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::String(self.next_symbol.clone())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for SetSharesTokenSymbol {
        const NAME: &'static str = "setSharesTokenSymbol";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SignalMigration {
        pub vault_proxy: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
        pub bypass_failure: bool,
    }

    impl SignalMigration {
        const METHOD_ID: [u8; 4] = [209u8, 95u8, 155u8, 156u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Bool,
                ],
                maybe_data.unwrap(),
            )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                bypass_failure: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.vault_proxy)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.next_vault_accessor)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.next_vault_lib)),
                ethabi::Token::Bool(self.bypass_failure),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }

    impl substreams_ethereum::Function for SignalMigration {
        const NAME: &'static str = "signalMigration";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
}

/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CurrentFundDeployerSet {
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
    }

    impl CurrentFundDeployerSet {
        const TOPIC_ID: [u8; 32] = [
            68u8, 207u8, 52u8, 162u8, 108u8, 203u8, 170u8, 138u8, 32u8, 230u8, 9u8, 138u8, 18u8,
            120u8, 226u8, 134u8, 130u8, 87u8, 46u8, 201u8, 146u8, 30u8, 180u8, 61u8, 141u8, 171u8,
            39u8, 74u8, 80u8, 62u8, 146u8, 245u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                prev_fund_deployer: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for CurrentFundDeployerSet {
        const NAME: &'static str = "CurrentFundDeployerSet";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationCancelled {
        pub vault_proxy: Vec<u8>,
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
        pub executable_timestamp: substreams::scalar::BigInt,
    }

    impl MigrationCancelled {
        const TOPIC_ID: [u8; 32] = [
            152u8, 81u8, 168u8, 242u8, 50u8, 195u8, 128u8, 235u8, 44u8, 16u8, 101u8, 117u8, 103u8,
            118u8, 227u8, 125u8, 133u8, 212u8, 254u8, 12u8, 140u8, 180u8, 0u8, 91u8, 13u8, 114u8,
            199u8, 73u8, 191u8, 239u8, 146u8, 62u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_proxy' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                prev_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                executable_timestamp: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }

    impl substreams_ethereum::Event for MigrationCancelled {
        const NAME: &'static str = "MigrationCancelled";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationExecuted {
        pub vault_proxy: Vec<u8>,
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
        pub executable_timestamp: substreams::scalar::BigInt,
    }

    impl MigrationExecuted {
        const TOPIC_ID: [u8; 32] = [
            173u8, 111u8, 179u8, 211u8, 110u8, 44u8, 218u8, 20u8, 141u8, 253u8, 67u8, 32u8, 60u8,
            26u8, 64u8, 192u8, 218u8, 154u8, 155u8, 170u8, 102u8, 42u8, 233u8, 214u8, 195u8, 176u8,
            195u8, 228u8, 169u8, 214u8, 166u8, 53u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_proxy' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                prev_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                executable_timestamp: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }

    impl substreams_ethereum::Event for MigrationExecuted {
        const NAME: &'static str = "MigrationExecuted";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationInCancelHookFailed {
        pub failure_return_data: Vec<u8>,
        pub vault_proxy: Vec<u8>,
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
    }

    impl MigrationInCancelHookFailed {
        const TOPIC_ID: [u8; 32] = [
            124u8, 121u8, 7u8, 10u8, 165u8, 26u8, 219u8, 103u8, 111u8, 185u8, 200u8, 208u8, 247u8,
            161u8, 248u8, 65u8, 255u8, 156u8, 245u8, 20u8, 68u8, 75u8, 55u8, 201u8, 82u8, 151u8,
            237u8, 28u8, 82u8, 84u8, 123u8, 50u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() < 128usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Bytes,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_proxy' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                prev_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                failure_return_data: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for MigrationInCancelHookFailed {
        const NAME: &'static str = "MigrationInCancelHookFailed";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationOutHookFailed {
        pub failure_return_data: Vec<u8>,
        pub hook: substreams::scalar::BigInt,
        pub vault_proxy: Vec<u8>,
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
    }

    impl MigrationOutHookFailed {
        const TOPIC_ID: [u8; 32] = [
            191u8, 10u8, 230u8, 131u8, 8u8, 131u8, 189u8, 137u8, 133u8, 95u8, 46u8, 123u8, 23u8,
            162u8, 146u8, 75u8, 93u8, 224u8, 178u8, 179u8, 212u8, 168u8, 131u8, 77u8, 124u8, 44u8,
            13u8, 93u8, 197u8, 52u8, 71u8, 219u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() < 160usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Bytes,
                    ethabi::ParamType::Uint(8usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_proxy' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                prev_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                failure_return_data: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
                hook: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for MigrationOutHookFailed {
        const NAME: &'static str = "MigrationOutHookFailed";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationSignaled {
        pub vault_proxy: Vec<u8>,
        pub prev_fund_deployer: Vec<u8>,
        pub next_fund_deployer: Vec<u8>,
        pub next_vault_accessor: Vec<u8>,
        pub next_vault_lib: Vec<u8>,
        pub executable_timestamp: substreams::scalar::BigInt,
    }

    impl MigrationSignaled {
        const TOPIC_ID: [u8; 32] = [
            181u8, 25u8, 196u8, 47u8, 219u8, 136u8, 248u8, 63u8, 9u8, 136u8, 20u8, 129u8, 63u8,
            46u8, 191u8, 241u8, 79u8, 191u8, 20u8, 243u8, 230u8, 139u8, 5u8, 78u8, 90u8, 152u8,
            131u8, 76u8, 105u8, 213u8, 75u8, 251u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                vault_proxy: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_proxy' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                prev_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_vault_lib: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                executable_timestamp: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }

    impl substreams_ethereum::Event for MigrationSignaled {
        const NAME: &'static str = "MigrationSignaled";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MigrationTimelockSet {
        pub prev_timelock: substreams::scalar::BigInt,
        pub next_timelock: substreams::scalar::BigInt,
    }

    impl MigrationTimelockSet {
        const TOPIC_ID: [u8; 32] = [
            72u8, 28u8, 226u8, 138u8, 17u8, 69u8, 165u8, 217u8, 181u8, 158u8, 45u8, 29u8, 126u8,
            186u8, 51u8, 188u8, 51u8, 80u8, 18u8, 77u8, 239u8, 143u8, 220u8, 145u8, 3u8, 34u8,
            56u8, 211u8, 67u8, 173u8, 83u8, 97u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                prev_timelock: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                next_timelock: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }

    impl substreams_ethereum::Event for MigrationTimelockSet {
        const NAME: &'static str = "MigrationTimelockSet";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NominatedOwnerRemoved {
        pub nominated_owner: Vec<u8>,
    }

    impl NominatedOwnerRemoved {
        const TOPIC_ID: [u8; 32] = [
            228u8, 19u8, 101u8, 132u8, 165u8, 232u8, 187u8, 133u8, 214u8, 99u8, 1u8, 243u8, 206u8,
            139u8, 17u8, 204u8, 46u8, 90u8, 139u8, 234u8, 57u8, 121u8, 241u8, 14u8, 162u8, 69u8,
            19u8, 83u8, 132u8, 154u8, 204u8, 71u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() != 0usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Ok(Self {
                nominated_owner: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'nominated_owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for NominatedOwnerRemoved {
        const NAME: &'static str = "NominatedOwnerRemoved";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NominatedOwnerSet {
        pub nominated_owner: Vec<u8>,
    }

    impl NominatedOwnerSet {
        const TOPIC_ID: [u8; 32] = [
            19u8, 44u8, 25u8, 222u8, 144u8, 27u8, 180u8, 237u8, 132u8, 3u8, 195u8, 71u8, 52u8,
            182u8, 200u8, 65u8, 213u8, 220u8, 87u8, 116u8, 88u8, 36u8, 218u8, 69u8, 42u8, 82u8,
            67u8, 53u8, 25u8, 234u8, 90u8, 191u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() != 0usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Ok(Self {
                nominated_owner: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'nominated_owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for NominatedOwnerSet {
        const NAME: &'static str = "NominatedOwnerSet";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct OwnershipTransferred {
        pub prev_owner: Vec<u8>,
        pub next_owner: Vec<u8>,
    }

    impl OwnershipTransferred {
        const TOPIC_ID: [u8; 32] = [
            139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
            164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8, 180u8,
            24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() != 0usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Ok(Self {
                prev_owner: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'prev_owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                next_owner: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[2usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'next_owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }

    impl substreams_ethereum::Event for OwnershipTransferred {
        const NAME: &'static str = "OwnershipTransferred";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SharesTokenSymbolSet {
        pub next_symbol: String,
    }

    impl SharesTokenSymbolSet {
        const TOPIC_ID: [u8; 32] = [
            240u8, 17u8, 87u8, 18u8, 194u8, 59u8, 96u8, 152u8, 85u8, 162u8, 141u8, 123u8, 137u8,
            99u8, 198u8, 209u8, 109u8, 115u8, 42u8, 227u8, 136u8, 177u8, 30u8, 93u8, 78u8, 198u8,
            213u8, 234u8, 228u8, 173u8, 134u8, 152u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                next_symbol: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }

    impl substreams_ethereum::Event for SharesTokenSymbolSet {
        const NAME: &'static str = "SharesTokenSymbolSet";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VaultProxyDeployed {
        pub fund_deployer: Vec<u8>,
        pub owner: Vec<u8>,
        pub vault_proxy: Vec<u8>,
        pub vault_lib: Vec<u8>,
        pub vault_accessor: Vec<u8>,
        pub fund_name: String,
    }

    impl VaultProxyDeployed {
        const TOPIC_ID: [u8; 32] = [
            165u8, 5u8, 96u8, 180u8, 72u8, 173u8, 121u8, 110u8, 246u8, 10u8, 223u8, 239u8, 254u8,
            28u8, 25u8, 237u8, 52u8, 218u8, 249u8, 76u8, 42u8, 166u8, 144u8, 93u8, 14u8, 32u8,
            144u8, 23u8, 219u8, 193u8, 159u8, 88u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() < 128usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::String,
                ],
                log.data.as_ref(),
            )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                fund_deployer: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[1usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'fund_deployer' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: ethabi::decode(&[ethabi::ParamType::Address], log.topics[2usize].as_ref())
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_lib: ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.topics[3usize].as_ref(),
                )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'vault_lib' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_proxy: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_accessor: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                fund_name: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }

    impl substreams_ethereum::Event for VaultProxyDeployed {
        const NAME: &'static str = "VaultProxyDeployed";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}
