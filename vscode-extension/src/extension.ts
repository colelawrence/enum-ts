"use strict";

import execa = require("execa");
import * as vscode from "vscode";
import enumTSBinary = require("enum-ts-bin");

const langs = ["typescript", "typescriptreact"];
const langSet = new Set(langs);

type Options = {
  bin: string;
};

const REGION_TO_FOLD = /#region enum-ts generated/g;

const FIX_COMMAND = "enum-ts.fix";
const FIX_ALL_COMMAND = "enum-ts.fix-all";

export function activate(_context: vscode.ExtensionContext) {
  function getEnumTSPath() {
    return enumTSBinary.getBinaryPath();
  }

  // 👎 formatter implemented as separate command
  vscode.commands.registerCommand(FIX_COMMAND, () => {
    const { activeTextEditor } = vscode.window;

    vscode.window.showErrorMessage(`enum-ts: ${getEnumTSPath()}`);

    if (activeTextEditor && langSet.has(activeTextEditor.document.languageId)) {
      const { document } = activeTextEditor;
      return editFormat({ bin: getEnumTSPath() }, document).then((enumEdit) => {
        if (enumEdit) {
          if (enumEdit.version === document.version) {
            const edit = new vscode.WorkspaceEdit();
            edit.replace(document.uri, enumEdit.range, enumEdit.replacement);
            return vscode.workspace.applyEdit(edit);
          } else {
            vscode.window.showErrorMessage(
              "enum-ts: Document changed while formatting"
            );
          }
        } else {
          vscode.window.showInformationMessage(
            "enum-ts: Already up-to-date based on hash. Delete regions to regenerate."
          );
        }
      });
    }
  });

  vscode.window.onDidChangeActiveTextEditor(async (textEditor) => {
    const document = textEditor.document;
    if (langSet.has(document.languageId)) {
      const text = document.getText();
      let match: RegExpExecArray;
      const prevSelection = textEditor.selection;
      while (((match = REGION_TO_FOLD.exec(text)), match != null)) {
        const start = document.positionAt(match.index);
        // based on https://github.com/eramdam/fold-imports/blob/1fcd5a37c9d53749379d13747b9ac27660c4712e/src/helpers.ts#L82
        await vscode.commands.executeCommand("editor.fold", {
          levels: 1,
          selectionLines: [start.line],
        });
      }
      textEditor.selection = prevSelection;
    }
  });

  const ENUM_RE = /\bEnum\b/;

  vscode.languages.registerCodeActionsProvider(langs, {
    provideCodeActions(document, range, context, cancelToken) {
      if (range.start.isEqual(range.end)) {
        range = document.getWordRangeAtPosition(range.start);
      }

      if (ENUM_RE.test(document.getText(range))) {
        return [
          { command: FIX_COMMAND, title: "enum-ts: Regenerate Enum helpers" },
        ];
      }

      return [];
    },
  });

  // 👍 formatter implemented using API
  vscode.languages.registerDocumentFormattingEditProvider(langs, {
    provideDocumentFormattingEdits(
      document: vscode.TextDocument
    ): Thenable<vscode.TextEdit[]> {
      return editFormat(
        {
          bin: enumTSBinary.getBinaryPath(),
        },
        document
      ).then((enumEdit) => {
        if (enumEdit && enumEdit.version === document.version) {
          return [
            vscode.TextEdit.replace(enumEdit.range, enumEdit.replacement),
          ];
        } else {
          return [];
        }
      });
    },
  });

  vscode.commands.registerCommand(FIX_ALL_COMMAND, () => {
    const paths = [];
    if (vscode.workspace.rootPath) {
      paths.push(vscode.workspace.rootPath);
    } else {
      paths.push(
        ...vscode.workspace.textDocuments
          .filter((textDoc) => langSet.has(textDoc.languageId))
          .map((textDoc) => textDoc.fileName)
      );
    }

    if (!paths.length) {
      vscode.window.showInformationMessage(
        "enum-ts: Unable to find any files to update"
      );
      return;
    }

    return fixAll({
      bin: getEnumTSPath(),
      fixPaths: paths,
    });
  });
}

function fixAll(options: { bin: string; fixPaths: string[] }): Promise<any> {
  return execa.command(
    `${options.bin} --write ${options.fixPaths
      .map((path) => JSON.stringify(path))
      .join(" ")}`
  );
}

const HAS_EDIT_RE = /update-range: L(\d+):(\d+)-L(\d+):(\d+)/;
function editFormat(
  options: Options,
  doc: vscode.TextDocument
): Promise<{
  version: number;
  range: vscode.Range;
  replacement: string;
} | null> {
  const versionAtStart = doc.version;
  return execa
    .command(`${options.bin} --edit-l1c0`, {
      stdin: "pipe",
      input: doc.getText(),
    })
    .then((done) => {
      const match = HAS_EDIT_RE.exec(done.stderr);
      if (match) {
        const startLine = parseInt(match[1]) - 1;
        const startCharacter = parseInt(match[2]);
        const endLine = parseInt(match[3]) - 1;
        const endCharacter = parseInt(match[4]);
        return {
          range: new vscode.Range(
            startLine,
            startCharacter,
            endLine,
            endCharacter
          ),
          version: versionAtStart,
          replacement: done.stdout,
        };
      } else {
        return null;
      }
    });
}
