import React from 'react';
import { createRoot } from 'react-dom/client';

import { Workbench } from './workbench';

const rootEle = document.getElementById('app');

if (rootEle) {
  const root = createRoot(rootEle);
  root.render(<Workbench />);
}
