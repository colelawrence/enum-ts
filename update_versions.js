// @ts-check
const fs = require("fs");

const TAG_RE = /^v?(\d+\.\d+\.\d+)$/;
const tagInput = process.argv[2];
const tagMatch = TAG_RE.exec(tagInput);
if (!tagMatch) {
  throw new Error(`Unrecognized tag argument "${tagInput}"`);
}
const tagSansV = tagMatch[1];

const updatedFiles = [];
updateFile("Cargo.toml", true, (current) =>
  current.replace(/\nversion = "[^\"]+"/, `\nversion = "${tagSansV}"`)
);
updateFile("packages/enum-ts-bin/package.json", true, (current) =>
  current.replace(/"version": "[^\"]+",/, `"version": "${tagSansV}",`)
);
updateFile("README.md", false, (current) =>
  current.replace(
    /(```typescript \/\/generated\(([\w\-]+)\))[\s\S]*?(```)/g,
    function (_, opener, name, closer) {
      const testFileSource = fs.readFileSync(
        __dirname + "/tests/" + name + ".ts",
        "utf8"
      );
      const generated = testFileSource.match(
        /enum-ts generated <\w+>([\s\S]+?)\/\/#endregion/
      );
      console.assert(generated, `generated section was matched in ${name}`);
      return `${opener}${generated[1]}${closer}`;
    }
  )
);
console.log(`Updated versions, now just create a commit, tag, and push tag:

cargo check && \\
git add Cargo.lock ${updatedFiles.join(" ")} && \\
git commit -m "Release v${tagSansV}" && \\
git tag v${tagSansV} && \\
git push origin v${tagSansV} && \\
echo "Done!"
`);

/**
 * @param {string} relativeFilePath
 * @param {boolean} mustUpdate
 * @param {(contents: string) => (string | null)} updater
 */
function updateFile(relativeFilePath, mustUpdate, updater) {
  const filePath = __dirname + "/" + relativeFilePath;
  const fileContents = fs.readFileSync(filePath, "utf8");
  const toWrite = updater(fileContents);
  if (!toWrite || (toWrite === fileContents && mustUpdate))
    throw new Error(`Failed to update ${relativeFilePath}.`);
  fs.writeFileSync(filePath, toWrite);
  updatedFiles.push(filePath);
  console.log(`Wrote: ${filePath}`);
}
