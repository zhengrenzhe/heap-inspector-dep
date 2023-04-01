import { Button } from '@mui/joy';
import React from 'react';

export function Workbench() {
  const onClick = async () => {
    const picker = (await showOpenFilePicker({ multiple: false }))[0];
    if (!picker) return;

    const stream = new TextDecoderStream();
    const reader = (await picker.getFile())
      .stream()
      .pipeThrough(stream)
      .getReader();

    let unFinished = true;
    while (unFinished) {
      const { done, value } = await reader.read();

      unFinished = !done;
    }
  };

  return (
    <div>
      <Button color="primary" variant="soft" size="sm" onClick={onClick}>
        Hello world
      </Button>
    </div>
  );
}
