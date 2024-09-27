import assert from "assert";
import {buildSpeech} from "@nmemonica/voice-ja";
import fs from "node:fs"
import { dirname, normalize, sep } from "node:path";
import { fileURLToPath } from "node:url";

describe("@nmemonica/voice-ja", function () {
    const dir = dirname(fileURLToPath(import.meta.url))
    const root = normalize(`${dir}${sep}..`);
    
    const voice_model = fs.readFileSync(`${root}${sep}models${sep}hts_voice_nitech_jp_atr503_m001-1.05${sep}nitech_jp_atr503_m001.htsvoice`)

    describe("buildSpeech throws", function () {
        it("missing query", function () {
            const result = () => {buildSpeech()};
            assert.throws(result,"query is a required parameter")
        });

        it("missing voice_model", function () {
            const result = () => {buildSpeech("テスト")};
            assert.throws(result,"voice_model is a required parameter")
        });
    });
    
    describe("buildSpeech", function () {
        it("byteLength", function () {
            const result = buildSpeech("テスト",voice_model);
            // console.log(JSON.stringify({byteLength:result.byteLength}))
            
            assert.equal(result.byteLength,147404)
        });
    });
});


