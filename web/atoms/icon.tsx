import React from "react";

// https://microsoft.github.io/vscode-codicons/dist/codicon.html
export function Icon(props: { name: string; size?: number }) {
  return (
    <span
      className={`codicon codicon-${props.name}`}
      style={{ fontSize: props.size ?? 22 }}
    />
  );
}
