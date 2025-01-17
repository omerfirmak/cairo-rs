## Cairo-VM Changelog

#### Upcoming Changes

* BREAKING CHANGE: refactor `Program` to optimize `Program::clone` [#999](https://github.com/lambdaclass/cairo-rs/pull/999)
    * Breaking change: many fields that were (unnecessarily) public become hidden by the refactor.

* BREAKING CHANGE: Add _builtin suffix to builtin names e.g.: output -> output_builtin [#1005](https://github.com/lambdaclass/cairo-rs/pull/1005)

* Implement hint on uint384_extension lib [#983](https://github.com/lambdaclass/cairo-rs/pull/983)

    `BuiltinHintProcessor` now supports the following hint:
    
    ```python
        def split(num: int, num_bits_shift: int, length: int):
            a = []
            for _ in range(length):
                a.append( num & ((1 << num_bits_shift) - 1) )
                num = num >> num_bits_shift
            return tuple(a)

        def pack(z, num_bits_shift: int) -> int:
            limbs = (z.d0, z.d1, z.d2)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        def pack_extended(z, num_bits_shift: int) -> int:
            limbs = (z.d0, z.d1, z.d2, z.d3, z.d4, z.d5)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        a = pack_extended(ids.a, num_bits_shift = 128)
        div = pack(ids.div, num_bits_shift = 128)

        quotient, remainder = divmod(a, div)

        quotient_split = split(quotient, num_bits_shift=128, length=6)

        ids.quotient.d0 = quotient_split[0]
        ids.quotient.d1 = quotient_split[1]
        ids.quotient.d2 = quotient_split[2]
        ids.quotient.d3 = quotient_split[3]
        ids.quotient.d4 = quotient_split[4]
        ids.quotient.d5 = quotient_split[5]

        remainder_split = split(remainder, num_bits_shift=128, length=3)
        ids.remainder.d0 = remainder_split[0]
        ids.remainder.d1 = remainder_split[1]
        ids.remainder.d2 = remainder_split[2]
    ```

* Add missing `\n` character in traceback string [#997](https://github.com/lambdaclass/cairo-rs/pull/997)
    * BugFix: Add missing `\n` character after traceback lines when the filename is missing ("Unknown Location")

* 0.11 Support
    * Add missing hints on cairo_secp lib [#991](https://github.com/lambdaclass/cairo-rs/pull/991):
        `BuiltinHintProcessor` now supports the following hints:
        ```python
        from starkware.cairo.common.cairo_secp.secp_utils import pack
        from starkware.python.math_utils import div_mod, safe_div

        N = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141
        x = pack(ids.x, PRIME) % N
        s = pack(ids.s, PRIME) % N
        value = res = div_mod(x, s, N)
        ```
        and: 
        ```python
        value = k = safe_div(res * s - x, N)
        ```
    * Layouts update [#874](https://github.com/lambdaclass/cairo-rs/pull/874)
    * Keccak builtin updated [#873](https://github.com/lambdaclass/cairo-rs/pull/873), [#883](https://github.com/lambdaclass/cairo-rs/pull/883)
    * Changes to `ec_op` [#876](https://github.com/lambdaclass/cairo-rs/pull/876)
    * Poseidon builtin [#875](https://github.com/lambdaclass/cairo-rs/pull/875)
    * Renamed Felt to Felt252 [#899](https://github.com/lambdaclass/cairo-rs/pull/899)
    * Added SegmentArenaBuiltinRunner [#913](https://github.com/lambdaclass/cairo-rs/pull/913)
    * Added `program_segment_size` argument to `verify_secure_runner` & `run_from_entrypoint` [#928](https://github.com/lambdaclass/cairo-rs/pull/928)
    * Added dynamic layout [#879](https://github.com/lambdaclass/cairo-rs/pull/879)
    * `get_segment_size` was exposed [#934](https://github.com/lambdaclass/cairo-rs/pull/934)

* Add missing hint on cairo_secp lib [#996](https://github.com/lambdaclass/cairo-rs/pull/996):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.python.math_utils import div_mod
        value = x_inv = div_mod(1, x, SECP_P)
    ```

* Add missing hints on cairo_secp lib [#994](https://github.com/lambdaclass/cairo-rs/pull/994)::

    `BuiltinHintProcessor` now supports the following hints:
    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import pack
        from starkware.python.math_utils import div_mod, safe_div

        a = pack(ids.a, PRIME)
        b = pack(ids.b, PRIME)
        value = res = div_mod(a, b, N)
    ```

    ```python
        value = k_plus_one = safe_div(res * b - a, N) + 1
    ```

* Add missing hint on cairo_secp lib [#992](https://github.com/lambdaclass/cairo-rs/pull/992):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import pack

        q, r = divmod(pack(ids.val, PRIME), SECP_P)
        assert r == 0, f"verify_zero: Invalid input {ids.val.d0, ids.val.d1, ids.val.d2}."
        ids.q = q % PRIME
    ```

* Add missing hint on cairo_secp lib [#990](https://github.com/lambdaclass/cairo-rs/pull/990):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import pack

        slope = pack(ids.slope, PRIME)
        x = pack(ids.point.x, PRIME)
        y = pack(ids.point.y, PRIME)

        value = new_x = (pow(slope, 2, SECP_P) - 2 * x) % SECP_P
    ```

* Add missing hint on cairo_secp lib [#989](https://github.com/lambdaclass/cairo-rs/pull/989):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import SECP_P
        q, r = divmod(pack(ids.val, PRIME), SECP_P)
        assert r == 0, f"verify_zero: Invalid input {ids.val.d0, ids.val.d1, ids.val.d2}."
        ids.q = q % PRIME
    ```

* Add missing hint on cairo_secp lib [#986](https://github.com/lambdaclass/cairo-rs/pull/986):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import SECP_P, pack
        from starkware.python.math_utils import div_mod

        # Compute the slope.
        x = pack(ids.pt.x, PRIME)
        y = pack(ids.pt.y, PRIME)
        value = slope = div_mod(3 * x ** 2, 2 * y, SECP_P)
    ```

* Add missing hint on cairo_secp lib [#984](https://github.com/lambdaclass/cairo-rs/pull/984):

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.cairo_secp.secp_utils import SECP_P, pack
        from starkware.python.math_utils import div_mod

        # Compute the slope.
        x0 = pack(ids.pt0.x, PRIME)
        y0 = pack(ids.pt0.y, PRIME)
        x1 = pack(ids.pt1.x, PRIME)
        y1 = pack(ids.pt1.y, PRIME)
        value = slope = div_mod(y0 - y1, x0 - x1, SECP_P)
    ```

* Implement hints on uint384 lib (Part 2) [#971](https://github.com/lambdaclass/cairo-rs/pull/971)

    `BuiltinHintProcessor` now supports the following hint:

    ```python
        memory[ap] = 1 if 0 <= (ids.a.d2 % PRIME) < 2 ** 127 else 0
    ```

 * Add alternative hint code for hint on _block_permutation used by 0.10.3 whitelist [#958](https://github.com/lambdaclass/cairo-rs/pull/958)

     `BuiltinHintProcessor` now supports the following hint:

    ```python
        from starkware.cairo.common.keccak_utils.keccak_utils import keccak_func
        _keccak_state_size_felts = int(ids.KECCAK_STATE_SIZE_FELTS)
        assert 0 <= _keccak_state_size_felts < 100

        output_values = keccak_func(memory.get_range(
            ids.keccak_ptr - _keccak_state_size_felts, _keccak_state_size_felts))
        segments.write_arg(ids.keccak_ptr, output_values)
    ```

* Make  hints code `src/hint_processor/builtin_hint_processor/hint_code.rs` public [#988](https://github.com/lambdaclass/cairo-rs/pull/988)

* Implement hints on uint384 lib (Part 1) [#960](https://github.com/lambdaclass/cairo-rs/pull/960)

    `BuiltinHintProcessor` now supports the following hints:

    ```python
        def split(num: int, num_bits_shift: int, length: int):
        a = []
        for _ in range(length):
            a.append( num & ((1 << num_bits_shift) - 1) )
            num = num >> num_bits_shift
        return tuple(a)

        def pack(z, num_bits_shift: int) -> int:
            limbs = (z.d0, z.d1, z.d2)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        a = pack(ids.a, num_bits_shift = 128)
        div = pack(ids.div, num_bits_shift = 128)
        quotient, remainder = divmod(a, div)

        quotient_split = split(quotient, num_bits_shift=128, length=3)
        assert len(quotient_split) == 3

        ids.quotient.d0 = quotient_split[0]
        ids.quotient.d1 = quotient_split[1]
        ids.quotient.d2 = quotient_split[2]

        remainder_split = split(remainder, num_bits_shift=128, length=3)
        ids.remainder.d0 = remainder_split[0]
        ids.remainder.d1 = remainder_split[1]
        ids.remainder.d2 = remainder_split[2]
    ```

    ```python
        ids.low = ids.a & ((1<<128) - 1)
        ids.high = ids.a >> 128
    ```

    ```python
            sum_d0 = ids.a.d0 + ids.b.d0
        ids.carry_d0 = 1 if sum_d0 >= ids.SHIFT else 0
        sum_d1 = ids.a.d1 + ids.b.d1 + ids.carry_d0
        ids.carry_d1 = 1 if sum_d1 >= ids.SHIFT else 0
        sum_d2 = ids.a.d2 + ids.b.d2 + ids.carry_d1
        ids.carry_d2 = 1 if sum_d2 >= ids.SHIFT else 0
    ```

    ```python
        def split(num: int, num_bits_shift: int, length: int):
            a = []
            for _ in range(length):
                a.append( num & ((1 << num_bits_shift) - 1) )
                num = num >> num_bits_shift
            return tuple(a)

        def pack(z, num_bits_shift: int) -> int:
            limbs = (z.d0, z.d1, z.d2)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        def pack2(z, num_bits_shift: int) -> int:
            limbs = (z.b01, z.b23, z.b45)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        a = pack(ids.a, num_bits_shift = 128)
        div = pack2(ids.div, num_bits_shift = 128)
        quotient, remainder = divmod(a, div)

        quotient_split = split(quotient, num_bits_shift=128, length=3)
        assert len(quotient_split) == 3

        ids.quotient.d0 = quotient_split[0]
        ids.quotient.d1 = quotient_split[1]
        ids.quotient.d2 = quotient_split[2]

        remainder_split = split(remainder, num_bits_shift=128, length=3)
        ids.remainder.d0 = remainder_split[0]
        ids.remainder.d1 = remainder_split[1]
        ids.remainder.d2 = remainder_split[2]
    ```

    ```python
        from starkware.python.math_utils import isqrt

        def split(num: int, num_bits_shift: int, length: int):
            a = []
            for _ in range(length):
                a.append( num & ((1 << num_bits_shift) - 1) )
                num = num >> num_bits_shift
            return tuple(a)

        def pack(z, num_bits_shift: int) -> int:
            limbs = (z.d0, z.d1, z.d2)
            return sum(limb << (num_bits_shift * i) for i, limb in enumerate(limbs))

        a = pack(ids.a, num_bits_shift=128)
        root = isqrt(a)
        assert 0 <= root < 2 ** 192
        root_split = split(root, num_bits_shift=128, length=3)
        ids.root.d0 = root_split[0]
        ids.root.d1 = root_split[1]
        ids.root.d2 = root_split[2]
    ```

* Re-export the `cairo-felt` crate as `cairo_vm::felt` [#981](https://github.com/lambdaclass/cairo-rs/pull/981)
  * Removes the need of explicitly importing `cairo-felt` in downstream projects
  and helps ensure there is no version mismatch caused by that

* Implement hint on `uint256_mul_div_mod`[#957](https://github.com/lambdaclass/cairo-rs/pull/957)

    `BuiltinHintProcessor` now supports the following hint:

    ```python
    a = (ids.a.high << 128) + ids.a.low
    b = (ids.b.high << 128) + ids.b.low
    div = (ids.div.high << 128) + ids.div.low
    quotient, remainder = divmod(a * b, div)

    ids.quotient_low.low = quotient & ((1 << 128) - 1)
    ids.quotient_low.high = (quotient >> 128) & ((1 << 128) - 1)
    ids.quotient_high.low = (quotient >> 256) & ((1 << 128) - 1)
    ids.quotient_high.high = quotient >> 384
    ids.remainder.low = remainder & ((1 << 128) - 1)
    ids.remainder.high = remainder >> 128"
    ```

    Used by the common library function `uint256_mul_div_mod`

#### [0.3.0-rc1] - 2023-04-13
* Derive Deserialize for ExecutionResources [#922](https://github.com/lambdaclass/cairo-rs/pull/922)
* Remove builtin names from VirtualMachine.builtin_runners [#921](https://github.com/lambdaclass/cairo-rs/pull/921)
* Implemented hints on common/ec.cairo [#888](https://github.com/lambdaclass/cairo-rs/pull/888)
* Changed `Memory.insert` argument types [#902](https://github.com/lambdaclass/cairo-rs/pull/902)
* feat: implemented `Deserialize` on Program by changing builtins field type to enum [#896](https://github.com/lambdaclass/cairo-rs/pull/896)
* Effective size computation from the VM exposed [#887](https://github.com/lambdaclass/cairo-rs/pull/887)
* Wasm32 Support! [#828](https://github.com/lambdaclass/cairo-rs/pull/828), [#893](https://github.com/lambdaclass/cairo-rs/pull/893)
* `MathError` added for math operation [#855](https://github.com/lambdaclass/cairo-rs/pull/855)
* Check for overflows in relocatable operations [#859](https://github.com/lambdaclass/cairo-rs/pull/859)
* Use `Relocatable` instead of `&MaybeRelocatable` in `load_data` and `get_range`[#860](https://github.com/lambdaclass/cairo-rs/pull/860) [#867](https://github.com/lambdaclass/cairo-rs/pull/867)
* Memory-related errors moved to `MemoryError` [#854](https://github.com/lambdaclass/cairo-rs/pull/854)
    * Removed unused error variants
    * Moved memory-related error variants to `MemoryError`
    * Changed memory getters to return `MemoryError` instead of `VirtualMachineError`
    * Changed all memory-related errors in hint from `HintError::Internal(VmError::...` to `HintError::Memory(MemoryError::...`
* feat: Builder pattern for `VirtualMachine` [#820](https://github.com/lambdaclass/cairo-rs/pull/820)
* Simplified `Memory::get` return type to `Option` [#852](https://github.com/lambdaclass/cairo-rs/pull/852)
* Improved idenitifier variable error handling [#851](https://github.com/lambdaclass/cairo-rs/pull/851)
* `CairoRunner::write_output` now prints missing and relocatable values [#853](https://github.com/lambdaclass/cairo-rs/pull/853)
* `VirtualMachineError::FailedToComputeOperands` error message expanded [#848](https://github.com/lambdaclass/cairo-rs/pull/848)
* Builtin names made public [#849](https://github.com/lambdaclass/cairo-rs/pull/849)
* `secure_run` flag moved to `CairoRunConfig` struct [#832](https://github.com/lambdaclass/cairo-rs/pull/832)
* `vm_core` error types revised and iimplemented `AddAssign` for `Relocatable` [#837](https://github.com/lambdaclass/cairo-rs/pull/837)
* `to_bigint` and `to_biguint` deprecated [#757](https://github.com/lambdaclass/cairo-rs/pull/757)
* `Memory` moved into `MemorySegmentManager` [#830](https://github.com/lambdaclass/cairo-rs/pull/830)
    * To reduce the complexity of the VM's memory and enforce proper usage (as the memory and its segment manager are now a "unified" entity)
    * Removed `memory` field from `VirtualMachine`
    * Added `memory` field to `MemorySegmentManager`
    * Removed `Memory` argument from methods where `MemorySegmentManager` is also an argument
    * Added test macro `segments` (an extension of the `memory` macro)
* `Display` trait added to Memory struct [#812](https://github.com/lambdaclass/cairo-rs/pull/812)
* feat: Extensible VirtualMachineError and removed PartialEq trait [#783](https://github.com/lambdaclass/cairo-rs/pull/783)
    * `VirtualMachineError::Other(anyhow::Error)` was added to allow to returning custom errors when using `cairo-rs`
    * The `PartialEq` trait was removed from the `VirtualMachineError` enum
* VM hooks added as a conditional feature [#761](https://github.com/lambdaclass/cairo-rs/pull/761)
    * Cairo-rs based testing tools such as cairo-foundry or those built by FuzzingLabs need access to the state of the VM at specific points during the execution.
    * This PR adds the possibility for users of the cairo-rs lib to execute their custom additional code during the program execution.
    * The Rust "feature" mechanism was used in order to guarantee that this ability is only available when the lib user needs it, and is not compiled when it's not required.
    * Three hooks were created:
        * before the first step
        * before each step
        * after each step
* ExecutionResource operations: add and substract [#774](https://github.com/lambdaclass/cairo-rs/pull/774), multiplication [#908](https://github.com/lambdaclass/cairo-rs/pull/908) , and `AddAssign` [#914](https://github.com/lambdaclass/cairo-rs/pull/914)

* Move `Memory` into `MemorySegmentManager` [#830](https://github.com/lambdaclass/cairo-rs/pull/830)
    * Structural changes:
        * Remove `memory: Memory` field from `VirtualMachine`
        * Add `memory: Memory` field to `MemorySegmentManager`
    * As a result of this, multiple public methods' signatures changed:
        * `BuiltinRunner` (and its inner enum types):
            * `initialize_segments(&mut self, segments: &mut MemorySegmentManager, memory: &mut Memory)` -> `initialize_segments(&mut self, segments: &mut MemorySegmentManager)`
            * `final_stack(&mut self, segments: &MemorySegmentManager, memory: &Memory, stack_pointer: Relocatable) -> Result<Relocatable, RunnerError>` -> `final_stack(&mut self, segments: &MemorySegmentManager, stack_pointer: Relocatable) -> Result<Relocatable, RunnerError>`
        * `MemorySegmentManager`
            * `add(&mut self, memory: &mut Memory) -> Relocatable` -> `add(&mut self) -> Relocatable`
            * `add_temporary_segment(&mut self, memory: &mut Memory) -> Relocatable` -> `add_temporary_segment(&mut self) -> Relocatable`
            * `load_data(&mut self, memory: &mut Memory, ptr: &MaybeRelocatable, data: &Vec<MaybeRelocatable>) -> Result<MaybeRelocatable, MemoryError>` -> `load_data(&mut self, ptr: &MaybeRelocatable, data: &Vec<MaybeRelocatable>) -> Result<MaybeRelocatable, MemoryError>`
            * `compute_effective_sizes(&mut self, memory: &Memory) -> &Vec<usize>` -> `compute_effective_sizes(&mut self) -> &Vec<usize>`
            * `gen_arg(&mut self, arg: &dyn Any, memory: &mut Memory) -> Result<MaybeRelocatable, VirtualMachineError>` -> `gen_arg(&mut self, arg: &dyn Any) -> Result<MaybeRelocatable, VirtualMachineError>`
            * `gen_cairo_arg(&mut self, arg: &CairoArg, memory: &mut Memory) -> Result<MaybeRelocatable, VirtualMachineError>` -> `gen_cairo_arg(&mut self, arg: &CairoArg) -> Result<MaybeRelocatable, VirtualMachineError>`
            * `write_arg(&mut self, memory: &mut Memory, ptr: &Relocatable, arg: &dyn Any) -> Result<MaybeRelocatable, MemoryError>` -> `write_arg(&mut self, ptr: &Relocatable, arg: &dyn Any) -> Result<MaybeRelocatable, MemoryError>`

* Refactor `Memory::relocate memory` [#784](https://github.com/lambdaclass/cairo-rs/pull/784)
    * Bugfixes:
        * `Memory::relocate_memory` now moves data in the temporary memory relocated by a relocation rule to the real memory
    * Aditional Notes:
        * When relocating temporary memory produces clashes with pre-existing values in the real memory, an InconsistentMemory error is returned instead of keeping the last inserted value. This differs from the original implementation.

* Restrict addresses to Relocatable + fix some error variants used in signature.rs [#792](https://github.com/lambdaclass/cairo-rs/pull/792)
    * Public Api Changes:
        * Change `ValidationRule` inner type to `Box<dyn Fn(&Memory, &Relocatable) -> Result<Vec<Relocatable>, MemoryError>>`.
        * Change `validated_addresses` field of `Memory` to `HashSet<Relocatable>`.
        * Change `validate_memory_cell(&mut self, address: &MaybeRelocatable) -> Result<(), MemoryError>` to `validate_memory_cell(&mut self, addr: &Relocatable) -> Result<(), MemoryError>`.

* Add `VmException` to `CairoRunner::run_from_entrypoint`[#775](https://github.com/lambdaclass/cairo-rs/pull/775)
    * Public Api Changes:
        * Change error return type of `CairoRunner::run_from_entrypoint` to `CairoRunError`.
        * Convert `VirtualMachineError`s outputed during the vm run to `VmException` in `CairoRunner::run_from_entrypoint`.
        * Make `VmException` fields public

* Fix `BuiltinRunner::final_stack` and remove quick fix [#778](https://github.com/lambdaclass/cairo-rs/pull/778)
    * Public Api changes:
        * Various changes to public `BuiltinRunner` method's signatures:
            * `final_stack(&self, vm: &VirtualMachine, pointer: Relocatable) -> Result<(Relocatable, usize), RunnerError>` to `final_stack(&mut self, segments: &MemorySegmentManager, memory: &Memory, pointer: Relocatable) -> Result<Relocatable,RunnerError>`.
            * `get_used_cells(&self, vm: &VirtualMachine) -> Result<usize, MemoryError>` to  `get_used_cells(&self, segments: &MemorySegmentManager) -> Result<usize, MemoryError>`.
            * `get_used_instances(&self, vm: &VirtualMachine) -> Result<usize, MemoryError>` to `get_used_instances(&self, segments: &MemorySegmentManager) -> Result<usize, MemoryError>`.
    * Bugfixes:
        * `BuiltinRunner::final_stack` now updates the builtin's stop_ptr instead of returning it. This replaces the bugfix on PR #768.

#### [0.1.3] - 2023-01-26
* Add secure_run flag + integrate verify_secure_runner into cairo-run [#771](https://github.com/lambdaclass/cairo-rs/pull/777)
    * Public Api changes:
        * Add command_line argument `secure_run`
        * Add argument `secure_run: Option<bool>` to `cairo_run`
        * `verify_secure_runner` is now called inside `cairo-run` when `secure_run` is set to true or when it not set and the run is not on `proof_mode`
    * Bugfixes:
        * `EcOpBuiltinRunner::deduce_memory_cell` now checks that both points are on the curve instead of only the first one
        * `EcOpBuiltinRunner::deduce_memory_cell` now returns the values of the point coordinates instead of the indices when a `PointNotOnCurve` error is returned

* Refactor `Refactor verify_secure_runner` [#768](https://github.com/lambdaclass/cairo-rs/pull/768)
    * Public Api changes:
        * Remove builtin name from the return value of `BuiltinRunner::get_memory_segment_addresses`
        * Simplify the return value of `CairoRunner::get_builtin_segments_info` to `Vec<(usize, usize)>`
        * CairoRunner::read_return_values now receives a mutable reference to VirtualMachine
    * Bugfixes:
        * CairoRunner::read_return_values now updates the `stop_ptr` of each builtin after calling `BuiltinRunner::final_stack`

* Use CairoArg enum instead of Any in CairoRunner::run_from_entrypoint [#686](https://github.com/lambdaclass/cairo-rs/pull/686)
    * Public Api changes:
        * Remove `Result` from `MaybeRelocatable::mod_floor`, it now returns a `MaybeRelocatable`
        * Add struct `CairoArg`
        * Change `arg` argument of `CairoRunner::run_from_entrypoint` from `Vec<&dyn Any>` to `&[&CairoArg]`
        * Remove argument `typed_args` from `CairoRunner::run_from_entrypoint`
        * Remove no longer used method `gen_typed_arg` from `VirtualMachine` & `MemorySegmentManager`
        * Add methods `MemorySegmentManager::gen_cairo_arg` & `MemorySegmentManager::write_simple_args` as typed counterparts to `MemorySegmentManager::gen_arg` & `MemorySegmentManager::write_arg`

#### [0.1.1] - 2023-01-11

* Add input file contents to traceback [#666](https://github.com/lambdaclass/cairo-rs/pull/666/files)
    * Public Api changes:
        * `VirtualMachineError` enum variants containing `MaybeRelocatable` and/or `Relocatable` values now use the `Display` format instead of `Debug` in their `Display` implementation
        * `get_traceback` now adds the source code line to each traceback entry
* Use hint location instead of instruction location when building VmExceptions from hint failure [#673](https://github.com/lambdaclass/cairo-rs/pull/673/files)
    * Public Api changes:
        * `hints` field added to `InstructionLocation`
        * `Program.instruction_locations` type changed from `Option<HashMap<usize, Location>>` to `Option<HashMap<usize, InstructionLocation>>`
        * `VirtualMachineError`s produced by `HintProcessor::execute_hint()` will be wrapped in a `VirtualMachineError::Hint` error containing their hint_index
        * `get_location()` now receives an an optional usize value `hint_index`, used to obtain hint locations
* Default implementation of compile_hint [#680](https://github.com/lambdaclass/cairo-rs/pull/680)
    * Internal changes:
        * Make the `compile_hint` implementation which was in the `BuiltinHintProcessor` the default implementation in the trait.
* Add new error type `HintError` [#676](https://github.com/lambdaclass/cairo-rs/pull/676)
    * Public Api changes:
        * `HintProcessor::execute_hint()` now returns a `HintError` instead of a `VirtualMachineError`
        * Helper functions on `hint_processor_utils.rs` now return a `HintError`
* Change the Dictionary used in dict hints to store MaybeRelocatable instead of BigInt [#687](https://github.com/lambdaclass/cairo-rs/pull/687)
    * Public Api changes:
        * `DictManager`, its dictionaries, and all dict module hints implemented in rust now use `MaybeRelocatable` for keys and values instead of `BigInt`
        * Add helper functions that allow extracting ids variables as `MaybeRelocatable`: `get_maybe_relocatable_from_var_name` & `get_maybe_relocatable_from_reference`
        * Change inner value type of dict-related `HintError` variants to `MaybeRelocatable`

* Implement `substitute_error_message_attribute_references` [#689] (https://github.com/lambdaclass/cairo-rs/pull/689)
    * Public Api changes:
        * Remove `error_message_attributes` field from `VirtualMachine`, and `VirtualMachine::new`
        * Add `flow_tracking_data` field to `Attribute`
        * `get_error_attr_value` now replaces the references in the error message with the corresponding cairo values.
        * Remove duplicated handling of error attribute messages leading to duplicated into in the final error display.
* Fix multiplicative inverse bug [#697](https://github.com/lambdaclass/cairo-rs/pull/697) [#698](https://github.com/lambdaclass/cairo-rs/pull/698). The VM was using integer division rather than prime field inverse when deducing `op0` or `op1` for the multiplication opcode

#### [0.1.0] - 2022-12-30
* Add traceback to VmException [#657](https://github.com/lambdaclass/cairo-rs/pull/657)
    * Public API changes:
        * `traceback` field added to `VmException` struct
        * `pub fn from_vm_error(runner: &CairoRunner, error: VirtualMachineError, pc: usize) -> Self` is now `pub fn from_vm_error(runner: &CairoRunner, vm: &VirtualMachine, error: VirtualMachineError) -> Self`
        * `pub fn get_location(pc: &usize, runner: &CairoRunner) -> Option<Location>` is now `pub fn get_location(pc: usize, runner: &CairoRunner) -> Option<Location>`
        * `pub fn decode_instruction(encoded_instr: i64, mut imm: Option<BigInt>) -> Result<instruction::Instruction, VirtualMachineError>` is now `pub fn decode_instruction(encoded_instr: i64, mut imm: Option<&BigInt>) -> Result<instruction::Instruction, VirtualMachineError>`
        * `VmExcepion` field's string format now mirror their cairo-lang conterparts.
