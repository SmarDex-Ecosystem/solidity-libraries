// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import { Test } from "forge-std-1/Test.sol";

/// @custom:feature Fixture with utilities for testing the `HugeUint` uint512 library.
contract HugeUintFixture is Test {
    /**
     * @notice Call the test_utils rust command via vm.ffi.
     * @dev You need to run `cargo build -r` at the root of the repo before executing your test.
     * @param commandName The name of the command to call.
     * @param parameter The parameter for the command.
     */
    function vmFFIRustCommand(string memory commandName, string memory parameter) internal returns (bytes memory) {
        return vmFFIRustCommand(commandName, parameter, "", "");
    }

    /**
     * @notice Call the test_utils rust command via vm.ffi.
     * @dev You need to run `cargo build -r` at the root of the repo before executing your test.
     * @param commandName The name of the command to call.
     * @param parameter1 The first parameter for the command.
     * @param parameter2 The second parameter for the command.
     */
    function vmFFIRustCommand(string memory commandName, string memory parameter1, string memory parameter2)
        internal
        returns (bytes memory)
    {
        return vmFFIRustCommand(commandName, parameter1, parameter2, "");
    }

    /**
     * @notice Call the test_utils rust command via vm.ffi.
     * @dev You need to run `cargo build -r` at the root of the repo before executing your test.
     * @param commandName The name of the command to call.
     * @param parameter1 The first parameter for the command.
     * @param parameter2 The second parameter for the command.
     * @param parameter3 The third parameter for the command.
     */
    function vmFFIRustCommand(
        string memory commandName,
        string memory parameter1,
        string memory parameter2,
        string memory parameter3
    ) internal returns (bytes memory result_) {
        return vmFFIRustCommand(commandName, parameter1, parameter2, parameter3, "");
    }

    /**
     * @notice Call the test_utils rust command via `vm.ffi`.
     * @dev You need to run `cargo build -r` at the root of the repo before executing your test.
     * @param commandName The name of the command to call.
     * @param parameter1 The first parameter for the command.
     * @param parameter2 The second parameter for the command.
     * @param parameter3 The third parameter for the command.
     * @param parameter4 The fourth parameter for the command.
     */
    function vmFFIRustCommand(
        string memory commandName,
        string memory parameter1,
        string memory parameter2,
        string memory parameter3,
        string memory parameter4
    ) internal returns (bytes memory result_) {
        string[] memory cmds = new string[](6);

        cmds[0] = "./target/release/test_utils";
        cmds[1] = commandName;
        cmds[2] = parameter1;
        cmds[3] = parameter2;
        cmds[4] = parameter3;
        cmds[5] = parameter4;

        // As of now, the first 3 arguments are always used
        uint8 usedParametersCount = 3;
        if (bytes(parameter2).length > 0) ++usedParametersCount;
        if (bytes(parameter3).length > 0) ++usedParametersCount;
        if (bytes(parameter4).length > 0) ++usedParametersCount;

        result_ = _vmFFIRustCommand(cmds, usedParametersCount);
    }

    /**
     * @notice Execute the given command.
     * @dev Will shrink the cmds array to a length of `argsCount`.
     * @param cmds The different parts of the command to execute.
     * @param argsCount The number of used parameters.
     */
    function _vmFFIRustCommand(string[] memory cmds, uint8 argsCount) private returns (bytes memory) {
        assembly ("memory-safe") {
            // shrink the array to avoid passing too many arguments to the command
            mstore(cmds, argsCount)
        }

        return vm.ffi(cmds);
    }
}
