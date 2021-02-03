const { Binary } = require("binary-install");
const os = require("os");
const version = require("./package.json").version;

function getBinary() {
  const platform = getReleaseAssetPlatformSuffix();
  const author = "colelawrence";
  const name = "enum-ts";
  const url = `https://github.com/${author}/${name}/releases/download/v${version}/${name}-${platform}.tar.gz`;
  return new Binary(name, url);
}

module.exports = {
  /** @returns {string} path to where the binary has been downloaded to */
  getBinaryPath() {
    return getBinary().binaryPath;
  },
  /**
   * Run binary with current process's args and exits process when done.
   * @returns {never}
   */
  run() {
    const binary = getBinary();
    binary.run();
  },
  install() {
    const binary = getBinary();
    return binary.install();
  },
};

function getReleaseAssetPlatformSuffix() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") {
    return "win64";
  }
  if (type === "Linux" && arch === "x64") {
    return "linux";
  }
  if (type === "Darwin" && arch === "x64") {
    return "macos";
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}
