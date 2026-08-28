const { spawnSync } = require("node:child_process");
const fs = require("node:fs");

const manifestLocation = "./pkg/package.json";
const noticeLocation = "./pkg/ThirdPartyNotice.txt";
const noticeJSONLocation = "./pkg/ThirdPartyNotice.json";
const testPkgLocation = "./tests/package.json";

const additionalFileToPackage = [
  "ThirdPartyNotice.txt",
  "ThirdPartyNotice.json",
];

function getAllDependencyLicenses() {
  const { status, stdout, stderr } = spawnSync("cargo", [
    "license",
    "--direct-deps-only",
    "-j",
  ]);

  if (status !== 0) {
    throw new Error(`Could not run cargo-license ${stderr.toString()}`);
  }

  const all = stdout.toString();

  const tohoku_f01 = {
    name: "htsvoice-tohoku-f01",
    version: "8e33060",
    authors:
      "Intelligent Communication Network (Ito-Nose) Laboratory, Tohoku University",
    repository: "https://github.com/icn-lab/htsvoice-tohoku-f01",
    licenseText: fs.readFileSync("./htsvoice/tohoku-f01/COPYRIGHT.txt", {
      encoding: "utf8",
    }),
    license: "CC-BY-4.0",
  };

  const licenses = [tohoku_f01, ...JSON.parse(all)];

  console.table(
    licenses
      .sort((a, b) => a.name.charCodeAt(0) - b.name.charCodeAt(0))
      .map((l) => ({
        name: l.name,
        version: l.version,
        license: l.license,
        authors: l.authors,
      })),
  );
  return licenses;
}

function packageAdditionalFiles() {
  const pkg = fs.readFileSync(manifestLocation, { encoding: "utf8" });
  const manifest = JSON.parse(pkg);
  manifest.files = Array.from(
    new Set([...manifest.files, ...additionalFileToPackage]),
  );

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

const licenseLinks = {
  MIT: "https://mit-license.org/",
  "BSD-3-Clause": "https://opensource.org/license/bsd-3-clause",
  "Apache-2.0": "https://www.apache.org/licenses/LICENSE-2.0",
  "CC-BY-4.0": "https://creativecommons.org/licenses/by/4.0/",
  ISC: "https://www.isc.org/licenses/",

  "Apache-2.0 OR MIT":
    "https://www.apache.org/licenses/LICENSE-2.0\nhttps://mit-license.org/",
};

function aggregateThirdPartyNotice() {
  const pkg = fs.readFileSync(manifestLocation, { encoding: "utf8" });
  const manifest = JSON.parse(pkg);

  const s =
    "================================================================================\n";

  const pkgDeps = getAllDependencyLicenses();
  const dependencies = pkgDeps.reduce((acc, dep) => {
    const { name, authors } = dep;

    if (manifest.name.endsWith(name) && manifest.collaborators[0] === authors) {
      return acc;
    }

    return [...acc, dep];
  }, []);

  const licenses = dependencies.reduce(
    (acc, info) => {
      const { name, license, version, authors, repository, licenseText } = info;

      if (license === undefined || license === null) {
        throw new Error(
          `Expected dependency to have license ${name} ${version}`,
        );
      }

      const licenceLink = licenseLinks[license];

      if (licenceLink === undefined) {
        throw new Error(`Required link for ${license}`);
      }

      const depLicenseChunk = `${s}- ${name} v${version} - ${authors}\n- ${
        repository
      }\n${s}\n${licenseText || license}\n${licenceLink}\n\n`;

      return acc + depLicenseChunk;
    },
    `THIRD PARTY SOFTWARE NOTICES AND INFORMATION
Do Not Translate or Localize
\n`,
  );

  // ThirdPartyNotice.txt
  fs.writeFileSync(noticeLocation, licenses, {
    encoding: "utf8",
  });

  // ThirdPartyNotice.json
  fs.writeFileSync(noticeJSONLocation, JSON.stringify(dependencies, null, 2), {
    encoding: "utf8",
  });
}

function updateTestsVersion() {
  const pkg = fs.readFileSync(manifestLocation, { encoding: "utf8" });
  const manifest = JSON.parse(pkg);
  const version = manifest.version;

  const test = fs.readFileSync(testPkgLocation, { encoding: "utf8" });
  const testManifest = JSON.parse(test);
  const [importType, testPkgVer] =
    testManifest.devDependencies[manifest.name].split(":");

  testManifest.devDependencies[manifest.name] =
    `${importType}:../pkg/${manifest.name.replace("@", "").replace("/", "-")}-${version}.tgz`;

  fs.writeFileSync(testPkgLocation, JSON.stringify(testManifest, null, 4), {
    encoding: "utf8",
  });
}

const args = process.argv.slice(2);

args.forEach((arg) => {
  switch (arg) {
    case "third-party":
      aggregateThirdPartyNotice();
      packageAdditionalFiles();
      break;
    case "test-update":
      updateTestsVersion();
      break;

    default:
      throw new Error(`Unexpected argument ${arg}`);
  }
});
