// script= "test":"node --experimental-wasm-modules ./wasm_js_test_quick.js",

import {buildSpeech} from "@nmemonica/voice-ja";
import { spawnSync } from "node:child_process";
import fs from "node:fs"
import { dirname, normalize, sep } from "node:path";
import { fileURLToPath } from "node:url";

const dir = dirname(fileURLToPath(import.meta.url))
const root = normalize(`${dir}${sep}..`);
const model_path = `${root}${sep}models${sep}hts_voice_nitech_jp_atr503_m001-1.05${sep}nitech_jp_atr503_m001.htsvoice`;

const voice_model = fs.readFileSync(model_path)
const result = buildSpeech("テスト", voice_model);

console.log(JSON.stringify({byteLength: result.byteLength}))
spawnSync("aplay", [], {input: result})