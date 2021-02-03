const TAG_RE = /^v?(\d+\.\d+\.\d+)$/;
const tagInput = process.argv[2];
const tagMatch = TAG_RE.exec(tagInput);
if (!tagMatch) {
  throw new Error(`Unrecognized tag argument "${tagInput}"`);
}
const tagSansV = tagMatch[1];

console.log(`
git tag -d v${tagSansV} && \\
git push --delete origin v${tagSansV}
`);
