use crate::stdlib::{collections::HashMap, prelude::*, sync::Arc};

use crate::{
    serde::deserialize_program::{
        deserialize_and_parse_program, Attribute, BuiltinName, HintParams, Identifier,
        InstructionLocation, ReferenceManager,
    },
    types::{errors::program_errors::ProgramError, relocatable::MaybeRelocatable},
};
use felt::Felt252;

#[cfg(feature = "std")]
use std::path::Path;

// NOTE: `Program` has been split in two containing some data that will be deep-copied
// and some that will be allocated on the heap inside an `Arc<_>`.
// This is because it has been reported that cloning the whole structure when creating
// a `CairoRunner` becomes a bottleneck, but the following solutions were tried and
// discarded:
// - Store only a reference in `CairoRunner` rather than cloning; this doesn't work
//   because then we need to introduce explicit lifetimes, which broke `cairo-rs-py`
//   since PyO3 doesn't support Python objects containing structures with lifetimes.
// - Directly pass an `Arc<Program>` to `CairoRunner::new()` and simply copy that:
//   there was a prohibitive performance hit of 10-15% when doing so, most likely
//   either because of branch mispredictions or the extra level of indirection going
//   through a random location on the heap rather than the likely-to-be-cached spot
//   on the stack.
//
// So, the compromise was to identify which data was less used and avoid copying that,
// using `Arc<_>`, while the most accessed fields remain on the stack for the main
// loop to access. The fields in `SharedProgramData` are either preprocessed and
// copied explicitly (_in addition_ to the clone of `Program`) or are used only in
// exceptional circumstances, such as when reconstructing a backtrace on execution
// failures.
// Fields in `Program` (other than `SharedProgramData` itself) are used by the main logic.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub(crate) struct SharedProgramData {
    pub builtins: Vec<BuiltinName>,
    pub data: Vec<MaybeRelocatable>,
    pub hints: HashMap<usize, Vec<HintParams>>,
    pub main: Option<usize>,
    //start and end labels will only be used in proof-mode
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub error_message_attributes: Vec<Attribute>,
    pub instruction_locations: Option<HashMap<usize, InstructionLocation>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub(crate) shared_program_data: Arc<SharedProgramData>,
    pub constants: HashMap<String, Felt252>,
    pub reference_manager: ReferenceManager,
    pub identifiers: HashMap<String, Identifier>,
}

impl Program {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        builtins: Vec<BuiltinName>,
        data: Vec<MaybeRelocatable>,
        main: Option<usize>,
        hints: HashMap<usize, Vec<HintParams>>,
        reference_manager: ReferenceManager,
        identifiers: HashMap<String, Identifier>,
        error_message_attributes: Vec<Attribute>,
        instruction_locations: Option<HashMap<usize, InstructionLocation>>,
    ) -> Result<Program, ProgramError> {
        let shared_program_data = SharedProgramData {
            builtins,
            data,
            hints,
            main,
            start: None,
            end: None,
            error_message_attributes,
            instruction_locations,
        };
        Ok(Self {
            shared_program_data: Arc::new(shared_program_data),
            constants: {
                let mut constants = HashMap::new();
                for (key, value) in identifiers.iter() {
                    if value.type_.as_deref() == Some("const") {
                        let value = value
                            .value
                            .clone()
                            .ok_or_else(|| ProgramError::ConstWithoutValue(key.clone()))?;
                        constants.insert(key.clone(), value);
                    }
                }

                constants
            },
            reference_manager,
            identifiers,
        })
    }

    #[cfg(feature = "std")]
    pub fn from_file(path: &Path, entrypoint: Option<&str>) -> Result<Program, ProgramError> {
        let file_content = std::fs::read(path)?;
        deserialize_and_parse_program(&file_content, entrypoint)
    }

    pub fn from_bytes(bytes: &[u8], entrypoint: Option<&str>) -> Result<Program, ProgramError> {
        deserialize_and_parse_program(bytes, entrypoint)
    }
}

impl Default for Program {
    fn default() -> Self {
        Self {
            shared_program_data: Arc::new(SharedProgramData::default()),
            constants: HashMap::new(),
            reference_manager: ReferenceManager {
                references: Vec::new(),
            },
            identifiers: HashMap::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::deserialize_program::{ApTracking, FlowTrackingData};
    use crate::utils::test_utils::*;
    use felt::felt_str;
    use num_traits::Zero;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new() {
        let reference_manager = ReferenceManager {
            references: Vec::new(),
        };

        let builtins: Vec<BuiltinName> = Vec::new();
        let data: Vec<MaybeRelocatable> = vec![
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(1000),
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(2000),
            mayberelocatable!(5201798304953696256),
            mayberelocatable!(2345108766317314046),
        ];

        let program = Program::new(
            builtins.clone(),
            data.clone(),
            None,
            HashMap::new(),
            reference_manager,
            HashMap::new(),
            Vec::new(),
            None,
        )
        .unwrap();

        assert_eq!(program.shared_program_data.builtins, builtins);
        assert_eq!(program.shared_program_data.data, data);
        assert_eq!(program.shared_program_data.main, None);
        assert_eq!(program.identifiers, HashMap::new());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new_program_with_identifiers() {
        let reference_manager = ReferenceManager {
            references: Vec::new(),
        };

        let builtins: Vec<BuiltinName> = Vec::new();

        let data: Vec<MaybeRelocatable> = vec![
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(1000),
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(2000),
            mayberelocatable!(5201798304953696256),
            mayberelocatable!(2345108766317314046),
        ];

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        identifiers.insert(
            String::from("__main__.main"),
            Identifier {
                pc: Some(0),
                type_: Some(String::from("function")),
                value: None,
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        identifiers.insert(
            String::from("__main__.main.SIZEOF_LOCALS"),
            Identifier {
                pc: None,
                type_: Some(String::from("const")),
                value: Some(Felt252::zero()),
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        let program = Program::new(
            builtins.clone(),
            data.clone(),
            None,
            HashMap::new(),
            reference_manager,
            identifiers.clone(),
            Vec::new(),
            None,
        )
        .unwrap();

        assert_eq!(program.shared_program_data.builtins, builtins);
        assert_eq!(program.shared_program_data.data, data);
        assert_eq!(program.shared_program_data.main, None);
        assert_eq!(program.identifiers, identifiers);
        assert_eq!(
            program.constants,
            [("__main__.main.SIZEOF_LOCALS", Felt252::zero())]
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect::<HashMap<_, _>>(),
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new_program_with_invalid_identifiers() {
        let reference_manager = ReferenceManager {
            references: Vec::new(),
        };

        let builtins: Vec<BuiltinName> = Vec::new();

        let data: Vec<MaybeRelocatable> = vec![
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(1000),
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(2000),
            mayberelocatable!(5201798304953696256),
            mayberelocatable!(2345108766317314046),
        ];

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        identifiers.insert(
            String::from("__main__.main"),
            Identifier {
                pc: Some(0),
                type_: Some(String::from("function")),
                value: None,
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        identifiers.insert(
            String::from("__main__.main.SIZEOF_LOCALS"),
            Identifier {
                pc: None,
                type_: Some(String::from("const")),
                value: None,
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        let program = Program::new(
            builtins,
            data,
            None,
            HashMap::new(),
            reference_manager,
            identifiers.clone(),
            Vec::new(),
            None,
        );

        assert!(program.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialize_program_test() {
        let program = Program::from_bytes(
            include_bytes!("../../cairo_programs/manually_compiled/valid_program_a.json"),
            Some("main"),
        )
        .unwrap();

        let builtins: Vec<BuiltinName> = Vec::new();
        let data: Vec<MaybeRelocatable> = vec![
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(1000),
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(2000),
            mayberelocatable!(5201798304953696256),
            mayberelocatable!(2345108766317314046),
        ];

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        identifiers.insert(
            String::from("__main__.main"),
            Identifier {
                pc: Some(0),
                type_: Some(String::from("function")),
                value: None,
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Args"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.Args".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.ImplicitArgs"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.ImplicitArgs".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Return"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.Return".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.SIZEOF_LOCALS"),
            Identifier {
                pc: None,
                type_: Some(String::from("const")),
                value: Some(Felt252::zero()),
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        assert_eq!(program.shared_program_data.builtins, builtins);
        assert_eq!(program.shared_program_data.data, data);
        assert_eq!(program.shared_program_data.main, Some(0));
        assert_eq!(program.identifiers, identifiers);
    }

    /// Deserialize a program without an entrypoint.
    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialize_program_without_entrypoint_test() {
        let program = Program::from_bytes(
            include_bytes!("../../cairo_programs/manually_compiled/valid_program_a.json"),
            None,
        )
        .unwrap();

        let builtins: Vec<BuiltinName> = Vec::new();

        let error_message_attributes: Vec<Attribute> = vec![Attribute {
            name: String::from("error_message"),
            start_pc: 379,
            end_pc: 381,
            value: String::from("SafeUint256: addition overflow"),
            flow_tracking_data: Some(FlowTrackingData {
                ap_tracking: ApTracking {
                    group: 14,
                    offset: 35,
                },
                reference_ids: HashMap::new(),
            }),
        }];

        let data: Vec<MaybeRelocatable> = vec![
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(1000),
            mayberelocatable!(5189976364521848832),
            mayberelocatable!(2000),
            mayberelocatable!(5201798304953696256),
            mayberelocatable!(2345108766317314046),
        ];

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        identifiers.insert(
            String::from("__main__.main"),
            Identifier {
                pc: Some(0),
                type_: Some(String::from("function")),
                value: None,
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Args"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.Args".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.ImplicitArgs"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.ImplicitArgs".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Return"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
                full_name: Some("__main__.main.Return".to_string()),
                members: Some(HashMap::new()),
                cairo_type: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.SIZEOF_LOCALS"),
            Identifier {
                pc: None,
                type_: Some(String::from("const")),
                value: Some(Felt252::zero()),
                full_name: None,
                members: None,
                cairo_type: None,
            },
        );

        assert_eq!(program.shared_program_data.builtins, builtins);
        assert_eq!(program.shared_program_data.data, data);
        assert_eq!(program.shared_program_data.main, None);
        assert_eq!(program.identifiers, identifiers);
        assert_eq!(
            program.shared_program_data.error_message_attributes,
            error_message_attributes
        )
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialize_program_constants_test() {
        let program = Program::from_bytes(
            include_bytes!("../../cairo_programs/manually_compiled/deserialize_constant_test.json"),
            Some("main"),
        )
        .unwrap();

        let constants = [
            ("__main__.compare_abs_arrays.SIZEOF_LOCALS", Felt252::zero()),
            (
                "starkware.cairo.common.cairo_keccak.packed_keccak.ALL_ONES",
                felt_str!(
                    "3618502788666131106986593281521497120414687020801267626233049500247285301247"
                ),
            ),
            (
                "starkware.cairo.common.cairo_keccak.packed_keccak.BLOCK_SIZE",
                Felt252::new(3),
            ),
            (
                "starkware.cairo.common.alloc.alloc.SIZEOF_LOCALS",
                felt_str!(
                    "-3618502788666131213697322783095070105623107215331596699973092056135872020481"
                ),
            ),
            (
                "starkware.cairo.common.uint256.SHIFT",
                felt_str!("340282366920938463463374607431768211456"),
            ),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<HashMap<_, _>>();

        assert_eq!(program.constants, constants);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn default_program() {
        let shared_program_data = SharedProgramData {
            builtins: Vec::new(),
            data: Vec::new(),
            hints: HashMap::new(),
            main: None,
            start: None,
            end: None,
            error_message_attributes: Vec::new(),
            instruction_locations: None,
        };
        let program = Program {
            shared_program_data: Arc::new(shared_program_data),
            constants: HashMap::new(),
            reference_manager: ReferenceManager {
                references: Vec::new(),
            },
            identifiers: HashMap::new(),
        };

        assert_eq!(program, Program::default());
    }
}
