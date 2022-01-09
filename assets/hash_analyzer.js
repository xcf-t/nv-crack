
export function checkSignature(data) {
    return data.toString(16).toUpperCase().substring(0, 4);
}

import { find_hash_origin } from "../pkg/nv_crack.js";

export function analyzeHash(output, hash) {
    output.value += `Trying to reach target: 0x${hash.toString(16).padStart(8, "0")} (${hash})\n`;

    let result = find_hash_origin(...window.config.versionMask, hash);

    if (result) {
        output.value += `Found v8 version candidate ${result}\n`;

        const node_versions = window.versionMap[result];

        if (node_versions) {
            output.value += `Possible Node.JS versions:\n${node_versions.map(version => " - " + version).join("\n")}\n`;
        } else {
            output.value += "No matching Node.JS versions found\n";
        }
    } else {
        output.value += "Found no version candidates\n";
    }
    output.value += "\n";
}