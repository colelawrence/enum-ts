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
updateFile("Cargo.toml", (current) =>
  current.replace(/\nversion = "[^\"]+"/, `\nversion = "${tagSansV}"`)
);
updateFile("packages/enum-ts-bin/package.json", (current) =>
  current.replace(/"version": "[^\"]+",/, `"version": "${tagSansV}",`)
);
console.log(`Updated versions, now just create a commit, tag, and push tag:

git add ${updatedFiles.join(" ")} && \\
git commit -m "Release v${tagSansV}" && \\
git tag v${tagSansV} && \\
git push origin v${tagSansV} && \\
echo "Done!"
`);

/**
 * @param {string} relativeFilePath
 * @param {(contents: string) => (string | null)} updater
 */
function updateFile(relativeFilePath, updater) {
  const filePath = __dirname + "/" + relativeFilePath;
  const fileContents = fs.readFileSync(filePath, "utf8");
  const toWrite = updater(fileContents);
  if (!toWrite || toWrite === fileContents)
    throw new Error(`Failed to update ${relativeFilePath}.`);
  fs.writeFileSync(filePath, toWrite);
  updatedFiles.push(filePath);
  console.log(`Wrote: ${filePath}`);
}
