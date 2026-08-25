const fs = require("node:fs");

const manifestLocation = "./pkg/package.json";
const noticeLocation = "./pkg/ThirdPartyNotice.txt";

function packageAdditionalFiles() {
  const files = ["ThirdPartyNotice.txt"];

  const pkg = fs.readFileSync(manifestLocation, { encoding: "utf8" });
  const manifest = JSON.parse(pkg);
  manifest.files = Array.from(new Set([...manifest.files, ...files]));

  console.table(
    manifest.files.reduce(
      (acc, file, i) => ({ ...acc, [i]: { "pkg includes": file } }),
      {},
    ),
  );
  fs.writeFileSync(manifestLocation, JSON.stringify(manifest, null, 2), {
    encoding: "utf8",
  });
}

function aggregateThirdPartyNotice() {
  const tohokuHeader = `--------------------------------------------------------------------------------
htsvoice-tohoku-f01 v8e33060 - 2015 Intelligent Communication Network (Ito-Nose) Laboratory, Tohoku University
https://github.com/icn-lab/htsvoice-tohoku-f01
--------------------------------------------------------------------------------

`;
  const tohokuBody = fs.readFileSync("./htsvoice/tohoku-f01/COPYRIGHT.txt", {
    encoding: "utf8",
  });

  const notice =
    `THIRD PARTY SOFTWARE NOTICES AND INFORMATION
Do Not Translate or Localize

` +
    tohokuHeader +
    tohokuBody;

  fs.writeFileSync(noticeLocation, notice, {
    encoding: "utf8",
  });
}

aggregateThirdPartyNotice();
packageAdditionalFiles();
