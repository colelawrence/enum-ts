"use strict";

import execa = require("execa");
import * as vscode from "vscode";

const langs = ["typescript", "typescriptreact"];
const langSet = new Set(langs);

type Options = {
  enumTSExecutable: string;
};

export function activate(context: vscode.ExtensionContext) {
  // 👎 formatter implemented as separate command
  vscode.commands.registerCommand("enum-ts.fix", () => {
    const { activeTextEditor } = vscode.window;

    if (activeTextEditor && langSet.has(activeTextEditor.document.languageId)) {
      const { document } = activeTextEditor;
      return editFormat(
        {
          enumTSExecutable: "enum-ts",
        },
        document
      ).then((enumEdit) => {
        if (enumEdit && enumEdit.version === document.version) {
          const edit = new vscode.WorkspaceEdit();
          edit.replace(document.uri, enumEdit.range, enumEdit.replacement);
          return vscode.workspace.applyEdit(edit);
        }
      });
    }
  });

  // 👍 formatter implemented using API
  vscode.languages.registerDocumentFormattingEditProvider(langs, {
    provideDocumentFormattingEdits(
      document: vscode.TextDocument
    ): Thenable<vscode.TextEdit[]> {
      return editFormat(
        {
          enumTSExecutable: "enum-ts",
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
    .command(`${options.enumTSExecutable} --edit-l1c0`, {
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
