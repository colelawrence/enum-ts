"use strict";

import execa = require("execa");
import * as vscode from "vscode";
import enumTSBinary = require("enum-ts-bin");

const langs = ["typescript", "typescriptreact"];
const langSet = new Set(langs);

type Options = {
  enumTSExecutable: string;
};

const REGION_TO_FOLD = /#region enum-ts generated/g;

const FIX_COMMAND = "enum-ts.fix";

export function activate(context: vscode.ExtensionContext) {
  // 👎 formatter implemented as separate command
  vscode.commands.registerCommand(FIX_COMMAND, () => {
    const { activeTextEditor } = vscode.window;

    if (activeTextEditor && langSet.has(activeTextEditor.document.languageId)) {
      const { document } = activeTextEditor;
      return editFormat(
        {
          enumTSExecutable: "enum-ts",
        },
        document
      ).then((enumEdit) => {
        if (enumEdit) {
          if (enumEdit.version === document.version) {
            const edit = new vscode.WorkspaceEdit();
            edit.replace(document.uri, enumEdit.range, enumEdit.replacement);
            return vscode.workspace.applyEdit(edit);
          } else {
            vscode.window.showErrorMessage("Document changed while formatting");
          }
        } else {
          vscode.window.showInformationMessage(
            "Already up-to-date based on hash. Delete regions to regenerate."
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
          enumTSExecutable: enumTSBinary.getBinaryPath(),
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
