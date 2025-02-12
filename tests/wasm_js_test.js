import assert from "assert";
import { buildSpeech } from "@nmemonica/voice-ja";

describe("@nmemonica/voice-ja", function () {
  describe("throws if", function () {
    it("missing query", function () {
      const result = () => {
        buildSpeech("", 0);
      };
      assert.throws(result, "query is a required parameter");
    });
  });

  describe("buildSpeech", function () {
    it("default voice", function () {
      const { buffer: result } = buildSpeech("", 0, "テスト");
      assert.equal(result.byteLength, 48148);
    });
    it("happy voice", function () {
      const { buffer: result } = buildSpeech("", 0, "テスト", "happy");
      assert.equal(result.byteLength, 50746);
    });
  });
});
