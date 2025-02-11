import assert from "assert";
import { buildSpeech } from "@nmemonica/voice-ja";
import fs from "node:fs";
import { dirname, normalize, sep } from "node:path";
import { fileURLToPath } from "node:url";

describe("@nmemonica/voice-ja", function () {
  const dir = dirname(fileURLToPath(import.meta.url));
  const root = normalize(`${dir}${sep}..`);

    describe("buildSpeech throws", function () {
        it("missing query", function () {
            const result = () => {buildSpeech("", 0)};
            assert.throws(result,"query is a required parameter")
        });

        it("missing voice_model", function () {
            const result = () => {buildSpeech("", 0, "テスト")};
            assert.throws(result,"voice_model is a required parameter")
        });
    });
    
    describe("buildSpeech", function () {
        it("byteLength", function () {
            const result = buildSpeech("", 0, "テスト", voice_model);
            // console.log(JSON.stringify({byteLength:result.byteLength}))
            
            assert.equal(result.byteLength,147404)
        });
    });
  });

  describe("buildSpeech", function () {
    it("byteLength", function () {
      const {
        uid,
        index,
        buffer: result,
      } = buildSpeech("", 0, "テスト", "neutral");
      // console.log(JSON.stringify({byteLength:result.byteLength}))

      assert.equal(result.byteLength, 48148);
    });

});
